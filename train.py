import sys
from pathlib import Path

import time
import torch
import torch.optim as optim


sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code

start_time = time.time()

class PolicyNetwork(torch.nn.Module):
    def __init__(self, state_size, action_size):
        super(PolicyNetwork, self).__init__()
        self.fc1 = torch.nn.Linear(state_size, 128)
        self.fc2 = torch.nn.Linear(128, 256)
        self.fc3 = torch.nn.Linear(256, 128)
        self.fc4 = torch.nn.Linear(128, action_size)

        torch.nn.init.xavier_uniform_(self.fc1.weight)
        torch.nn.init.xavier_uniform_(self.fc2.weight)
        torch.nn.init.xavier_uniform_(self.fc3.weight)
        torch.nn.init.uniform_(self.fc4.weight, -0.01, 0.01)

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = torch.relu(self.fc3(x))
        x = torch.softmax(self.fc4(x), dim=-1)
        return x

def choose_action(state, game_options, policy_net):
    # convert state to PyTorch tensor
    state = torch.tensor(state, dtype=torch.float32)
    
    # get Q values for each action using the policy network
    q_values = policy_net(state)

    # set unavailable actions' q_values to negative infinity
    for i, option in enumerate(game_options):
        if not option:
            q_values[i] = -float('inf')

    # choose action with the highest Q value
    action, action_idx = torch.argmax(q_values).item(), torch.argmax(q_values) 

    #print(f"Choosing action from {q_values}")


    return (action, action_idx)

def update_policy(policy_net, optimizer, rewards, log_probs):
    # calculate discounted rewards
    discounted_rewards = []
    cumulative_reward = 0
    for r in reversed(rewards):
        cumulative_reward = r + 0.99 * cumulative_reward
        discounted_rewards.insert(0, cumulative_reward)

    # normalize discounted rewards
    discounted_rewards = torch.tensor(discounted_rewards, dtype=torch.float32)
    discounted_rewards = (discounted_rewards - discounted_rewards.mean()) / (discounted_rewards.std() + 1e-5)

    # calculate loss
    policy_loss = []
    for log_prob, reward in zip(log_probs, discounted_rewards):
        policy_loss.append(-log_prob * reward)
    policy_loss = torch.stack(policy_loss).sum()

    # update policy network
    pol_loss = policy_loss
    gradf1 = policy_net.fc1.weight.grad
    gradf2 = policy_net.fc2.weight.grad
    gradf3 = policy_net.fc3.weight.grad    
    policy_losses.append(pol_loss)


    optimizer.zero_grad()
    # print(discounted_rewards)
    # print(log_probs)
    #policy_loss = policy_loss.clone().detach().requires_grad_(True)
    # print(policy_loss)
    test_qs = policy_net(torch.tensor(test_state, dtype=torch.float32))
    if any(test_qs.isnan()):
        print(f"Got NaNs before backward {test_qs}")
        print(f"Policy loss was {pol_loss}")
        exit(1)



    
    try:
        policy_loss.backward()

    except RuntimeError as e:
        if 'element of the differentiated inputs' in str(e):
            print("Error: policy_loss doesn't contain grad_fn")
            print(f"Policy loss was {pol_loss}")
            exit(1)
        else:
            raise e


    test_qs = policy_net(torch.tensor(test_state, dtype=torch.float32))
    if any(test_qs.isnan()):
        print(f"Got NaNs after backward {test_qs}")
        print(f"Rewards were: {rewards}")
        print(f"Log probs were: {log_probs}")
        print(f"Policy grad fc1 {policy_net.fc1.weight.grad}")
        print(f"Policy grad fc1 before backward {gradf1}")
        print(f"Policy grad fc2 {policy_net.fc2.weight.grad}")
        print(f"Policy grad fc2 before backward {gradf2}")
        print(f"Policy grad fc3 {policy_net.fc3.weight.grad}")
        print(f"Policy grad fc3 before backward {gradf3}")
        print(f"Policy loss was {pol_loss}")
        exit(1)

    # policy_loss.backward()
    optimizer.step()
    # torch.nn.utils.clip_grad_norm_(policy_net.parameters(), max_norm=1)

    test_qs = policy_net(torch.tensor(test_state, dtype=torch.float32))
    if any(test_qs.isnan()):
        print(f"Got NaNs after optimizer {test_qs}")
        # print(f"Rewards were: {rewards}")
        # print(f"Log probs were: {log_probs}")
        print(f"Policy grad fc1 {policy_net.fc1.weight.grad}")
        print(f"Policy grad fc1 before backward {gradf1}")
        print(f"Policy grad fc2 {policy_net.fc2.weight.grad}")
        print(f"Policy grad fc2 before backward {gradf2}")
        print(f"Policy grad fc3 {policy_net.fc3.weight.grad}")
        print(f"Policy grad fc3 before backward {gradf3}")
        print(f"Policy loss was {pol_loss}")
        print(f"All pol losses were {policy_losses}")
        exit(1)



def play_game(policy_net, optimizer):
    game = libsap.Game()
    log_probs = []
    rewards = []
    playing_game = True
    # play the game and collect rewards and log probabilities
    while playing_game:
        state = game.game_state()
        actions = game.game_options()
        action, action_idx = choose_action(state, actions, policy_net)

        q_vals = policy_net(torch.tensor(state, dtype=torch.float32))

        # print(f"Q vals are = {q_vals}")
        # print(sum(q_vals))

        selected_qval = q_vals[action]
        if selected_qval == 0:
            selected_qval = torch.tensor(1/85, dtype=torch.float32)
        log_prob = torch.log(selected_qval)

        # print(f"Selected Q- val = {selected_qval}")
        # print(f"Log Prob = {log_prob}")
        #log_prob = torch.log(policy_net(torch.tensor(state, dtype=torch.float32))[action])
        if selected_qval.isnan():
            print(f"Got NaN q value, q values are: {q_vals}")
            exit(1)


        reward = game.do_action(actions[action_idx])
        playing_game = game.game_alive()
        log_probs.append(log_prob)
        rewards.append(reward)

    # print(f"Rewards are {rewards}")
    #print(f"Log probs are {log_probs}")

    wins, lives = game.gen_game()
    if len(win_array) > 10:
        win_array.pop(0)
        win_array.append(wins)
    else:
        win_array.append(wins)
    print(f"Won {wins} games. {lives} remaining")

    # update policy network
    update_policy(policy_net, optimizer, rewards, log_probs)




pysap = libsap.Game()

# define game state size and number of actions
state_size = 66
num_actions = 85

# initialize policy network
policy_net = PolicyNetwork(state_size, num_actions)
optimizer = optim.Adam(policy_net.parameters(), lr = 0.00001)


test_game = libsap.Game()
test_state = test_game.game_state()

q_vals_before_training = policy_net(torch.tensor(test_state, dtype=torch.float32))
# print(f"Starting Q vals are = {q_vals_before_training}")

game_count = 0
win_array = []
average_wins_per_100 = []
policy_losses = []
wins = 0

for i in range(10000):
    game_count += 1
    play_game(policy_net, optimizer)
    print(f"Current Game Count = {game_count}")
    print("Win per 100 array")
    print(average_wins_per_100)
    wins += win_array[-1]
    if i % 100 == 0:
        average_wins_per_100.append(wins/100)
        wins = 0



q_vals_after_training = policy_net(torch.tensor(test_state, dtype=torch.float32))
# print(f"Final Q vals are = {q_vals_after_training}")


diff = q_vals_after_training - q_vals_before_training
print(f"Diff in Q vals are = {diff}")


# while sum(win_array) / 10 < 5:
#     game_count += 1
#     play_game(policy_net, optimizer)
#     print(f"Current Game Count = {game_count}")

#play_game(policy_net, optimizer)

print(f"The win array is: {win_array}")
end_time = time.time()
print("Code execution time:", end_time - start_time, "seconds")