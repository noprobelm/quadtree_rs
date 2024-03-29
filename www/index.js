import { Point, Rectangle, QuadTree } from "quadtree";

const canvas = document.getElementById("quadtree-canvas");
canvas.height = window.innerHeight;
canvas.width = window.innerWidth;
const ctx = canvas.getContext('2d');

const center = Point.new(Math.floor(canvas.width / 2), Math.floor(canvas.height / 2))
let qt = QuadTree.new(Rectangle.new(center, canvas.width, canvas.height), 4)

const c = Point.new(Math.floor(canvas.width / 2), Math.floor(canvas.height / 2))
const qt_boundary = Rectangle.new(c, canvas.width, canvas.height)

let isDrawing = false;
drawBorder();

canvas.addEventListener('mousedown', function(evt) {
    isDrawing = true;
    insertPoint(evt);
});

canvas.addEventListener('mouseup', function(evt) {
    isDrawing = false;
});

canvas.addEventListener('mousemove', function(evt) {
    if (isDrawing) {
        insertPoint(evt);
    }
});

function drawBorder() {
    ctx.fillStyle = "#315771";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

function getMousePos(canvas, evt) {
    const rect = canvas.getBoundingClientRect(); // Gets the canvas position relative to the viewport
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}

function insertPoint(evt) {
    const mousePos = getMousePos(canvas, evt);
    const point = Point.new(mousePos.x, mousePos.y);
    qt.insert(point);
    let data = JSON.parse(qt.query_all_for_js(qt_boundary));
    drawPoints(data['points']);
    drawRects(data['rects']);
}

function drawPoints(points) {
    ctx.fillStyle = "#A4BAB7";

    points.forEach(function(point) {
	ctx.beginPath();
	ctx.arc(point.x, point.y, 5, 0, 2 * Math.PI);
	ctx.fill();
    });
}

function drawRects(rects) {
    ctx.strokeStyle = "#F6AE2D"
    rects.forEach((rect) => {
	let x_start = rect.center.x - rect.width / 2
	let y_start = rect.center.y - rect.height / 2
	ctx.beginPath();
	ctx.strokeRect(x_start, y_start, rect.width, rect.height);
	ctx.stroke()
    });}
