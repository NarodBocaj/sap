import sys
from pathlib import Path

import time
import torch
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
    action = torch.argmax(q_values).item()

    return action

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
    state = np.random.rand(10)
    game_options = [[1, 2, 3], [4, 5, 6], [7, 8, 9], [], []]
    log_probs = []
    rewards = []

    # play the game and collect rewards and log probabilities
    while not done:
        action = choose_action(state, game_options, policy_net)
        log_prob = torch.log(policy_net(torch.tensor(state, dtype=torch.float32))[action])
        state, reward, done = take_action(action)
        log_probs.append(log_prob)
        rewards.append(reward)

    # update policy network
    update_policy(policy_net, optimizer, rewards, log_probs)

pysap = libsap.Game()

# define game state size and number of actions
state_size = 66
num_actions = 5

# initialize policy network
policy_net = PolicyNetwork(state_size, num_actions)

chosen_action = choose_action(game_state, game_options, policy_net)


end_time = time.time()
print("Code execution time:", end_time - start_time, "seconds")