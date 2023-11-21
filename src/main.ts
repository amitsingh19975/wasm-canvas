import { canvasMain } from './canvas';
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <canvas id="canvas" width="500" height="500"></canvas>
`

function main() {
    const canvas = document.getElementById('canvas') as HTMLCanvasElement;
    if (canvas == null) return;

    const width = window.innerWidth;
    const height = window.innerHeight;

    canvas.width = width - 20;
    canvas.height = height - 20;
    canvasMain('canvas');
}

requestAnimationFrame(main);

