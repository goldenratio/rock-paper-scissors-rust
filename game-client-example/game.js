import { generateUsername } from './username-genrator.js';

const apiPrefix = "http://localhost:8080/api"
function main() {
  let gameId = '';
  let playerName = generateUsername();
  let playerToken = '';

  let createGameButton = document.getElementById('create-game-btn');
  let joinButton = document.getElementById('join-game-btn');

  let gameIdTextEl = document.getElementById('game-id-txt');
  gameIdTextEl.value = '';

  let playerNameTextEl = document.getElementById('player-name-txt');
  playerNameTextEl.value = playerName;

  createGameButton.addEventListener('click', async () => {
    const response = await fetch(`${apiPrefix}/create`);
    const jsonData = await response.json();
    console.log('/create: ', JSON.stringify(jsonData));

    gameId = jsonData?.data?.gameId || '';
    gameIdTextEl.value = gameId;
  });

  joinButton.addEventListener('click', async () => {
    let jsonData = await sendPostRequest(`${apiPrefix}/join`, { gameId: gameIdTextEl.value, playerName: playerNameTextEl.value });
    console.log(`/join ${gameIdTextEl.value}: `, JSON.stringify(jsonData));
    playerToken = jsonData?.data?.playerToken || '';
  });

  [... document.getElementsByClassName('action-btn')].forEach(el => {
    el.addEventListener('click', async () => {
      const actionType = el.dataset.actionType;
      let jsonData = await sendPostRequest(`${apiPrefix}/game_action`, { actionType: actionType, gameId: gameIdTextEl.value, playerToken: playerToken });
      console.log(`/game_action ${actionType}: `, JSON.stringify(jsonData));
    });
  });
}

async function sendPostRequest(url, payload) {
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(payload)
  });
  return response.json();
}

main();
