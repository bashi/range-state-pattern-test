const RANGE_COLOR = '#eee';
const RANGE_COLOR_SELECTED = '#ccc';

export function invalidate() {
  const canvas = document.getElementById('range-canvas');
  const ctx = canvas.getContext('2d');
  ctx.clearRect(0, 0, canvas.width, canvas.height);
}

export function draw_selected_range(startX, endX) {
  const canvas = document.getElementById('range-canvas');
  const ctx = canvas.getContext('2d');
  const width = endX - startX;
  const height = canvas.height;

  ctx.strokeStyle = 'gray';
  ctx.strokeRect(startX, 0, width, height);
  ctx.fillStyle = RANGE_COLOR_SELECTED;
  ctx.fillRect(startX, 0, width, height);
}

export function draw_range(startX, endX) {
  const canvas = document.getElementById('range-canvas');
  const ctx = canvas.getContext('2d');
  const width = endX - startX;
  const height = canvas.height;

  ctx.strokeStyle = 'gray';
  ctx.strokeRect(startX, 0, width, height);
  ctx.fillStyle = RANGE_COLOR;
  ctx.fillRect(startX, 0, width, height);
}

export function get_width() {
  const canvas = document.getElementById('range-canvas');
  return canvas.width;
}

export function change_cursor(value) {
  const canvas = document.getElementById('range-canvas');
  canvas.style.cursor = value;
}