// beat-stream frontend – vanilla JS, no build tools
// ─────────────────────────────────────────────────────────────

(function () {
  "use strict";

  // ── Constants ─────────────────────────────────────────────
  const NUM_TRACKS = 4;
  const NUM_STEPS = 16;
  const BPM_MIN = 60;
  const BPM_MAX = 140;

  const TRACK_META = [
    { label: "BOOM 💥", color: "boom" },
    { label: "CRACK ⚡", color: "crack" },
    { label: "TICK ✨", color: "tick" },
    { label: "SNAP 👏", color: "snap" },
  ];

  // Preset patterns (4 tracks × 16 steps, 1 = active)
  const PRESETS = {
    Chill: [
      [1,0,0,0, 0,0,0,0, 1,0,0,0, 0,0,0,0],
      [0,0,0,0, 1,0,0,0, 0,0,0,0, 1,0,0,0],
      [0,0,1,0, 0,0,1,0, 0,0,1,0, 0,0,1,0],
      [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,1],
    ],
    Bounce: [
      [1,0,0,0, 0,0,1,0, 1,0,0,0, 0,0,1,0],
      [0,0,0,0, 1,0,0,0, 0,0,0,0, 1,0,0,0],
      [1,0,1,0, 1,0,1,0, 1,0,1,0, 1,0,1,0],
      [0,0,0,0, 0,0,0,1, 0,0,0,0, 0,0,0,1],
    ],
    Pulse: [
      [1,0,0,1, 0,0,1,0, 0,1,0,0, 1,0,0,0],
      [0,0,1,0, 0,0,0,0, 1,0,0,0, 0,0,1,0],
      [1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1],
      [0,0,0,0, 1,0,0,0, 0,0,0,0, 1,0,0,0],
    ],
    Sparse: [
      [1,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
      [0,0,0,0, 0,0,0,0, 1,0,0,0, 0,0,0,0],
      [0,0,0,0, 0,0,1,0, 0,0,0,0, 0,0,0,0],
      [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,1,0],
    ],
    Chaos: [
      [1,0,1,1, 0,1,0,1, 1,0,1,0, 0,1,1,0],
      [0,1,0,1, 1,0,1,0, 0,1,1,0, 1,0,0,1],
      [1,1,0,1, 0,1,1,0, 1,0,1,1, 0,1,0,1],
      [0,1,1,0, 1,0,0,1, 0,1,0,1, 1,0,1,0],
    ],
    Heartbeat: [
      [1,0,0,0, 0,0,1,0, 0,0,0,0, 0,0,0,0],
      [0,0,0,0, 0,0,0,0, 1,0,0,0, 0,0,0,0],
      [0,0,1,0, 0,0,0,0, 0,0,1,0, 0,0,0,0],
      [1,0,0,0, 0,0,1,0, 0,0,0,0, 0,0,0,0],
    ],
  };

  // ── DOM refs ──────────────────────────────────────────────
  const $grid        = document.getElementById("beat-grid");
  const $labels      = document.getElementById("grid-labels");
  const $playBtn     = document.getElementById("play-btn");
  const $bpmSlider   = document.getElementById("bpm-slider");
  const $bpmValue    = document.getElementById("bpm-value");
  const $connStatus  = document.getElementById("connection-status");
  const $roomId      = document.getElementById("room-id");
  const $userCount   = document.getElementById("user-count");
  const $startOverlay = document.getElementById("start-overlay");
  const $startBtn    = document.getElementById("start-btn");
  const $presetBtns  = document.querySelectorAll(".preset-btn");

  // ── State ─────────────────────────────────────────────────
  let grid = Array.from({ length: NUM_TRACKS }, () => new Array(NUM_STEPS).fill(false));
  let bpm = 120;
  let playing = false;
  let currentStep = -1;
  let ws = null;
  let reconnectAttempts = 0;
  const MAX_RECONNECT = 5;
  let reconnectTimer = null;
  let activePreset = null;
  let audioStarted = false;

  // ── Room ID ───────────────────────────────────────────────
  function getRoomId() {
    let id = location.hash.replace("#", "").trim();
    if (!id) {
      id = crypto.randomUUID ? crypto.randomUUID() : uuidFallback();
      location.hash = id;
    }
    return id;
  }

  function uuidFallback() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
      var r = (Math.random() * 16) | 0;
      return (c === "x" ? r : (r & 0x3) | 0x8).toString(16);
    });
  }

  const roomId = getRoomId();
  $roomId.textContent = roomId.slice(0, 8) + "…";
  $roomId.title = roomId;

  // ── Audio engine (Tone.js) ────────────────────────────────
  let kick, snare, hihat, clap;
  let sequenceId = null;

  function initAudio() {
    if (audioStarted) return;
    audioStarted = true;

    // Kick – membrane synth
    kick = new Tone.MembraneSynth({
      pitchDecay: 0.05,
      octaves: 6,
      oscillator: { type: "sine" },
      envelope: { attack: 0.001, decay: 0.3, sustain: 0, release: 0.1 },
    }).toDestination();

    // Snare – noise synth
    snare = new Tone.NoiseSynth({
      noise: { type: "white" },
      envelope: { attack: 0.001, decay: 0.15, sustain: 0, release: 0.05 },
    }).toDestination();

    // Hi-hat – metal synth
    hihat = new Tone.MetalSynth({
      frequency: 400,
      envelope: { attack: 0.001, decay: 0.06, release: 0.01 },
      harmonicity: 5.1,
      modulationIndex: 32,
      resonance: 4000,
      octaves: 1.5,
    }).toDestination();
    hihat.volume.value = -10;

    // Clap – noise burst with filter
    clap = new Tone.NoiseSynth({
      noise: { type: "pink" },
      envelope: { attack: 0.001, decay: 0.12, sustain: 0, release: 0.08 },
    }).toDestination();
    clap.volume.value = -4;

    Tone.getTransport().bpm.value = bpm;
    Tone.getTransport().loop = true;
    Tone.getTransport().loopStart = 0;
    Tone.getTransport().loopEnd = "1m";
  }

  function triggerSound(track) {
    switch (track) {
      case 0: kick.triggerAttackRelease("C1", "8n"); break;
      case 1: snare.triggerAttackRelease("8n"); break;
      case 2: hihat.triggerAttackRelease("C4", "32n"); break;
      case 3: clap.triggerAttackRelease("16n"); break;
    }
  }

  function startSequencer() {
    if (sequenceId !== null) return;

    Tone.getTransport().cancel();

    sequenceId = Tone.getTransport().scheduleRepeat((time) => {
      const step = Math.round(Tone.getTransport().position.split(":").reduce(
        (acc, v, i) => acc + parseFloat(v) * [16, 4, 1][i], 0
      )) % NUM_STEPS;

      // Use a simpler approach: track position via 16th notes
      Tone.getDraw().schedule(() => {
        updatePlayhead(step);
      }, time);

      for (let t = 0; t < NUM_TRACKS; t++) {
        if (grid[t][step]) {
          triggerSound(t);
        }
      }
    }, "16n");
  }

  function startPlayback() {
    if (!audioStarted) return;
    startSequencer();
    Tone.getTransport().start();
    playing = true;
    $playBtn.textContent = "⏸";
    $playBtn.classList.add("playing");
  }

  function stopPlayback() {
    Tone.getTransport().pause();
    playing = false;
    currentStep = -1;
    clearPlayhead();
    $playBtn.textContent = "▶";
    $playBtn.classList.remove("playing");
  }

  function togglePlayback() {
    if (!audioStarted) {
      Tone.start().then(() => {
        initAudio();
        startPlayback();
      });
      return;
    }
    playing ? stopPlayback() : startPlayback();
  }

  // ── Grid rendering ────────────────────────────────────────
  function buildGrid() {
    $grid.innerHTML = "";
    $labels.innerHTML = "";

    for (let t = 0; t < NUM_TRACKS; t++) {
      // Label
      const label = document.createElement("div");
      label.className = "grid-label";
      label.textContent = TRACK_META[t].label;
      $labels.appendChild(label);

      // Cells
      for (let s = 0; s < NUM_STEPS; s++) {
        const cell = document.createElement("div");
        cell.className = "cell";
        cell.dataset.track = t;
        cell.dataset.step = s;
        if (grid[t][s]) cell.classList.add("active");
        cell.addEventListener("click", () => onCellClick(t, s));
        $grid.appendChild(cell);
      }
    }
  }

  function getCell(track, step) {
    return $grid.querySelector(`.cell[data-track="${track}"][data-step="${step}"]`);
  }

  function setCellState(track, step, active, remote) {
    grid[track][step] = active;
    const cell = getCell(track, step);
    if (!cell) return;

    cell.classList.toggle("active", active);

    // Animation
    cell.classList.remove("bounce", "remote-pulse");
    void cell.offsetWidth; // reflow to restart animation
    cell.classList.add(remote ? "remote-pulse" : "bounce");
  }

  function syncGridFromState(tracks) {
    for (let t = 0; t < NUM_TRACKS; t++) {
      const steps = tracks[t] ? tracks[t].steps : new Array(NUM_STEPS).fill(false);
      for (let s = 0; s < NUM_STEPS; s++) {
        grid[t][s] = !!steps[s];
      }
    }
    refreshGridUI();
  }

  function refreshGridUI() {
    for (let t = 0; t < NUM_TRACKS; t++) {
      for (let s = 0; s < NUM_STEPS; s++) {
        const cell = getCell(t, s);
        if (cell) cell.classList.toggle("active", grid[t][s]);
      }
    }
  }

  // ── Playhead ──────────────────────────────────────────────
  function updatePlayhead(step) {
    if (step === currentStep) return;
    clearPlayhead();
    currentStep = step;
    for (let t = 0; t < NUM_TRACKS; t++) {
      const cell = getCell(t, step);
      if (cell) cell.classList.add("playhead");
    }
  }

  function clearPlayhead() {
    const cells = $grid.querySelectorAll(".cell.playhead");
    cells.forEach((c) => c.classList.remove("playhead"));
  }

  // ── Cell click → toggle ───────────────────────────────────
  function onCellClick(track, step) {
    // Optimistic toggle
    const newState = !grid[track][step];
    setCellState(track, step, newState, false);
    sendMessage({ type: "Toggle", track, step });
    clearActivePreset();
  }

  // ── BPM ───────────────────────────────────────────────────
  function setBpm(value) {
    bpm = Math.max(BPM_MIN, Math.min(BPM_MAX, value));
    $bpmSlider.value = bpm;
    $bpmValue.textContent = bpm;
    if (audioStarted) {
      Tone.getTransport().bpm.value = bpm;
    }
  }

  $bpmSlider.addEventListener("input", () => {
    const val = parseInt($bpmSlider.value, 10);
    setBpm(val);
    sendMessage({ type: "SetBpm", bpm: val });
  });

  // ── Presets ───────────────────────────────────────────────
  function loadPreset(name) {
    const pattern = PRESETS[name];
    if (!pattern) return;

    // Diff current grid vs preset and send toggles
    for (let t = 0; t < NUM_TRACKS; t++) {
      for (let s = 0; s < NUM_STEPS; s++) {
        const want = !!pattern[t][s];
        if (grid[t][s] !== want) {
          sendMessage({ type: "Toggle", track: t, step: s });
          // Optimistic update
          setCellState(t, s, want, false);
        }
      }
    }

    setActivePreset(name);
  }

  function setActivePreset(name) {
    activePreset = name;
    $presetBtns.forEach((btn) => {
      btn.classList.toggle("active", btn.dataset.pattern === name);
    });
  }

  function clearActivePreset() {
    activePreset = null;
    $presetBtns.forEach((btn) => btn.classList.remove("active"));
  }

  $presetBtns.forEach((btn) => {
    btn.addEventListener("click", () => loadPreset(btn.dataset.pattern));
  });

  // ── WebSocket ─────────────────────────────────────────────
  function wsUrl() {
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    return `${proto}//${location.host}/api/rooms/${roomId}/ws`;
  }

  function connect() {
    if (ws && (ws.readyState === WebSocket.OPEN || ws.readyState === WebSocket.CONNECTING)) {
      return;
    }

    ws = new WebSocket(wsUrl());

    ws.onopen = () => {
      reconnectAttempts = 0;
      setConnected(true);
      sendMessage({ type: "RequestState" });
    };

    ws.onmessage = (evt) => {
      let msg;
      try { msg = JSON.parse(evt.data); } catch { return; }
      handleServerMessage(msg);
    };

    ws.onclose = () => {
      setConnected(false);
      scheduleReconnect();
    };

    ws.onerror = () => {
      ws.close();
    };
  }

  function scheduleReconnect() {
    if (reconnectAttempts >= MAX_RECONNECT) return;
    const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), 16000);
    reconnectAttempts++;
    clearTimeout(reconnectTimer);
    reconnectTimer = setTimeout(connect, delay);
  }

  function sendMessage(msg) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(msg));
    }
  }

  function setConnected(connected) {
    $connStatus.textContent = connected ? "🟢" : "🔴";
    $connStatus.title = connected ? "Connected" : "Disconnected";
    $connStatus.classList.toggle("connected", connected);
    $connStatus.classList.toggle("disconnected", !connected);
  }

  // ── Server message handling ───────────────────────────────
  function handleServerMessage(msg) {
    switch (msg.type) {
      case "State":
        if (msg.room) {
          syncGridFromState(msg.room.tracks || []);
          setBpm(msg.room.bpm || 120);
          $userCount.textContent = "👤 " + (msg.room.active_users || 0);
        }
        break;

      case "Toggle":
        // Server confirmed a toggle (could be from us or another user)
        {
          const active = !grid[msg.track][msg.step];
          setCellState(msg.track, msg.step, active, !!msg.user_id);
          clearActivePreset();
        }
        break;

      case "BpmChanged":
        setBpm(msg.bpm);
        break;

      case "UserJoined":
        $userCount.textContent = "👤 " + (msg.count || 0);
        break;

      case "UserLeft":
        $userCount.textContent = "👤 " + (msg.count || 0);
        break;

      case "Error":
        console.warn("[beat-stream] Server error:", msg.message);
        break;
    }
  }

  // ── Start overlay ─────────────────────────────────────────
  $startBtn.addEventListener("click", () => {
    Tone.start().then(() => {
      initAudio();
      $startOverlay.classList.add("hidden");
      startPlayback();
    });
  });

  // ── Play button ───────────────────────────────────────────
  $playBtn.addEventListener("click", togglePlayback);

  // ── Init ──────────────────────────────────────────────────
  buildGrid();
  connect();
})();
