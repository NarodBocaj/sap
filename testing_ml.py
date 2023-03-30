import sys
from pathlib import Path

import time
import torch
import torch.optim as optim
import numpy as np



sys.path.append(str(Path(__file__).resolve().parent / "target" / "release"))
import libsap #my rust code

start_time = time.time()

class PolicyNetwork(torch.nn.Module):
    def __init__(self, state_size, action_size):
        super(PolicyNetwork, self).__init__()
        self.fc1 = torch.nn.Linear(state_size, 128)
        self.fc2 = torch.nn.Linear(128, 64)
        self.fc3 = torch.nn.Linear(64, action_size)

        torch.nn.init.xavier_uniform_(self.fc1.weight)
        torch.nn.init.xavier_uniform_(self.fc2.weight)
        torch.nn.init.xavier_uniform_(self.fc3.weight)

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = torch.softmax(self.fc3(x), dim=-1)
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
    discounted_rewards = (discounted_rewards - discounted_rewards.mean()) / (discounted_rewards.std() + 1e-9)

    # calculate loss
    policy_loss = []
    for log_prob, reward in zip(log_probs, discounted_rewards):
        policy_loss.append(-log_prob * reward)
    policy_loss = torch.stack(policy_loss).sum()

    # update policy network
    optimizer.zero_grad()
    policy_loss.backward()
    optimizer.step()

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
        log_prob = torch.log(policy_net(torch.tensor(state, dtype=torch.float32))[action])
        reward = game.do_action(actions[action_idx])
        playing_game = game.game_alive()
        log_probs.append(log_prob)
        rewards.append(reward)

        print(f"Rewards are {rewards}")
        print(f"Log probs are {log_probs}")

    wins, lives = game.gen_game()
    print(f"Won {wins} games. {lives} remaining")

    # update policy network
    update_policy(policy_net, optimizer, rewards, log_probs)

pysap = libsap.Game()

# define game state size and number of actions
state_size = 66
num_actions = 85

# initialize policy network
policy_net = PolicyNetwork(state_size, num_actions)
optimizer = optim.Adam(policy_net.parameters(), lr=0.01)

for _ in range(1):
    play_game(policy_net, optimizer)


#play_game(policy_net, optimizer)


end_time = time.time()
print("Code execution time:", end_time - start_time, "seconds")