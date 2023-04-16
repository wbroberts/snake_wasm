import init, { Game } from './pkg/snake_wasm.js';

const SIZE = 600;
const canvas = document.getElementById('snake-canvas');
const ctx = canvas.getContext('2d');

canvas.setAttribute('width', SIZE);
canvas.setAttribute('height', SIZE);

init().then(() => {
  const game = Game.new(ctx, 20, SIZE);
  addEventListener('keydown', e => game.control(e));

  const loop = () => {
    game.play();
    requestAnimationFrame(loop);
  };

  requestAnimationFrame(loop);
});
