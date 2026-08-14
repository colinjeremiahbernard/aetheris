// --- Configuration
const SSE_URL = "http://127.0.0.1:3000/telemetry/stream"; // change if needed
const MAX_POINTS = 30;

// --- DOM
const sensorEl = document.getElementById("sensor");
const valueEl = document.getElementById("value");
const severityEl = document.getElementById("severity");
const explanationEl = document.getElementById("explanation");
const lastUpdatedEl = document.getElementById("last-updated");
const statusEl = document.getElementById("status-indicator");
const sparkCanvas = document.getElementById("sparkline");
const ctx = sparkCanvas.getContext("2d");

// --- State
const points = []; // recent numeric values

function setStatus(connected) {
  if (connected) {
    statusEl.textContent = "Live";
    statusEl.classList.remove("disconnected");
    statusEl.classList.add("connected");
  } else {
    statusEl.textContent = "Disconnected";
    statusEl.classList.remove("connected");
    statusEl.classList.add("disconnected");
  }
}

function applySeverityClass(s) {
  severityEl.classList.remove("severity-normal","severity-high","severity-critical");
  if (!s) {
    severityEl.classList.add("severity-normal");
    return;
  }
  if (s === "Critical") severityEl.classList.add("severity-critical");
  else if (s === "High") severityEl.classList.add("severity-high");
  else severityEl.classList.add("severity-normal");
}

function formatTime(iso) {
  try { return new Date(iso).toLocaleString(); } catch(e) { return iso; }
}

function pushPoint(v) {
  if (v === null || v === undefined || Number.isNaN(v)) return;
  points.push(v);
  if (points.length > MAX_POINTS) points.shift();
  drawSparkline();
}

function drawSparkline(){
  const width = sparkCanvas.width = sparkCanvas.clientWidth * devicePixelRatio;
  const height = sparkCanvas.height = sparkCanvas.clientHeight * devicePixelRatio;
  ctx.clearRect(0,0,width,height);

  if (points.length === 0) return;

  // scale
  const min = Math.min(...points);
  const max = Math.max(...points);
  const pad = (max - min) * 0.1 || 1;
  const rmin = min - pad;
  const rmax = max + pad;
  const range = rmax - rmin;

  // path
  ctx.lineWidth = 2 * devicePixelRatio;
  ctx.strokeStyle = "#ffd166";
  ctx.beginPath();
  for (let i=0;i<points.length;i++){
    const x = (i / (points.length - 1 || 1)) * width;
    const y = height - ((points[i] - rmin) / range) * height;
    if (i===0) ctx.moveTo(x,y); else ctx.lineTo(x,y);
  }
  ctx.stroke();

  // fill under curve
  ctx.globalAlpha = 0.08;
  ctx.fillStyle = "#ffd166";
  ctx.lineTo(width, height);
  ctx.lineTo(0, height);
  ctx.closePath();
  ctx.fill();
  ctx.globalAlpha = 1;

  // draw min/max labels (dom)
  document.getElementById("min-val").textContent = `min ${min.toFixed(1)}`;
  document.getElementById("max-val").textContent = `max ${max.toFixed(1)}`;
}

// --- SSE
let evtSource;
function startSSE(){
  try {
    evtSource = new EventSource(SSE_URL);
  } catch(e){
    setStatus(false);
    lastUpdatedEl.textContent = "Unable to open EventSource.";
    return;
  }

  evtSource.addEventListener("telemetry", (e) => {
    try {
      const data = JSON.parse(e.data);

      // update main cards
      sensorEl.textContent = data.sensor_id || "—";
      valueEl.textContent = (data.value !== undefined) ? `${data.value} ${data.unit || ""}` : "—";
      severityEl.textContent = data.severity ?? "Normal";
      explanationEl.textContent = data.explanation || "—";

      applySeverityClass(data.severity);
      lastUpdatedEl.textContent = `Last update: ${formatTime(data.time)}`;
      setStatus(true);

      // sparkline
      pushPoint(Number(data.value));
    } catch(err) {
      console.error("parse error:", err);
    }
  });

  evtSource.onerror = () => {
    setStatus(false);
    lastUpdatedEl.textContent = "Stream disconnected.";
  };
}

// start
startSSE();

// reconnect logic
setInterval(()=>{
  if (!evtSource || evtSource.readyState === EventSource.CLOSED) {
    if (evtSource) evtSource.close();
    startSSE();
  }
}, 5000);