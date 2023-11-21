import { Canvas } from './wasm/pkg/wasm';

function getRelativeCoords(canvas: HTMLCanvasElement, x: number, y: number){
    const rect = canvas.getBoundingClientRect();
    if (rect == null) return {x, y};
    return {
        x: x - rect.left,
        y: y - rect.top,
    };
}

export function canvasMain(id: string) {
    const canvas = new Canvas(id, "black");
    const jsCanvas = canvas.get_js_canvas();
    if (jsCanvas == null) return;

    document.addEventListener('mousemove', (e) => {
        const {x, y} = getRelativeCoords(jsCanvas, e.clientX, e.clientY);
        canvas.on_mouse_move(x, y, Date.now());
    });

    document.addEventListener('mousedown', (e) => {
        const {x, y} = getRelativeCoords(jsCanvas, e.clientX, e.clientY);
        canvas.on_mouse_down(x, y, Date.now());
    });

    document.addEventListener('mouseup', (e) => {
        const {x, y} = getRelativeCoords(jsCanvas, e.clientX, e.clientY);
        canvas.on_mouse_up(x, y, Date.now());
    });

    document.addEventListener('keydown', (e) => {
        canvas.on_key_down(e.key, Date.now());
    });

    document.addEventListener('keyup', (e) => {
        canvas.on_key_up(e.key, Date.now());
    });
    
    document.addEventListener('keypress', (e) => {
        canvas.on_key_press(e.key, Date.now());
    });

    const ball = canvas.add_circle(500, 100, 50, {
        fill_color: 'red',
        stroke_width: 5,
        stroke_color: 'blue',
    });
    
    const bat = canvas.add_rect(100, 100, 15, 100, {
        fill_color: 'white',
    });

    const computer = canvas.add_rect(400, 100, 15, 100, {
        fill_color: 'white',
    });

    const text = canvas.add_text("Hello World", 431, 503, {
        font_size: 20,
        fill_color: 'white',
    });

    void ball;
    void bat;
    void computer;

    let x = 0;
    let y = 0;

    const renderFn = async () => {
        text.translate(x + 100, y + 50);
        text.rotation(25, "tl");
        canvas.render();

        // x += 1;
        // y += 1;
        requestAnimationFrame(renderFn);
    };

    requestAnimationFrame(renderFn)

    // const fn = () => {
    //     circle.translate(100 + x, 100 + y);
    //     canvas.render();
    //     x += 1;
    //     requestAnimationFrame(fn);
    // };
    // requestAnimationFrame(fn);
    // canvas.add_point(200, 200, {});
}