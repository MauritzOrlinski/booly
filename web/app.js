// doesitsat – one text box, one answer.
// The formula is parsed and Tseitin-transformed in logic.js; the CNF goes to
// the booly WebAssembly solver running in worker.js.
import { parse, tseitin, toDimacs, format, FormulaError } from "./logic.js";

const field = document.getElementById("formula");
const out = document.getElementById("out");
const engine = document.getElementById("engine");

// ───────────── worker ─────────────
let worker, workerReady;
const pending = new Map(); // id -> resolve
let nextId = 1;

function startWorker() {
  worker = new Worker("worker.js", { type: "module" });
  workerReady = new Promise((resolve, reject) => {
    worker.addEventListener("message", (e) => {
      if (e.data.type === "ready") {
        engine.textContent = "booly · CDCL SAT solver · Rust → WebAssembly, runs in your browser";
        resolve();
      } else if (e.data.type === "result") {
        pending.get(e.data.id)?.(e.data);
        pending.delete(e.data.id);
      }
    });
    worker.addEventListener("error", (e) => {
      engine.textContent = "The solver failed to load: " + e.message;
      reject(e);
    });
  });
}
startWorker();

function solve(dimacs) {
  const id = nextId++;
  return new Promise((resolve) => {
    pending.set(id, resolve);
    workerReady.then(() => worker.postMessage({ id, dimacs, options: { preprocess: true } }));
  });
}

function cancelAll() {
  worker.terminate();
  pending.clear();
  startWorker();
}

// ───────────── UI helpers ─────────────
const el = (tag, cls, text) => {
  const n = document.createElement(tag);
  if (cls) n.className = cls;
  if (text !== undefined) n.textContent = text;
  return n;
};

// Only the placeholder gets capitalised; variable names must stay as typed.
const cap = (s) => (s === "the formula" ? "The formula" : s);

function show(state, title, ...children) {
  out.classList.remove("stale");
  out.replaceChildren();
  const v = el("div", `verdict ${state}`);
  v.append(el("span", "dot"), el("span", "", title));
  out.append(v, ...children);
}

function assignment(names, model) {
  const wrap = el("div", "assign");
  const value = new Map(model.map((l) => [Math.abs(l), l > 0]));
  names.forEach((name, i) => {
    const v = value.get(i + 1) ?? true;
    wrap.append(el("span", v ? "t" : "f", `${name} = ${v}`));
  });
  return wrap;
}

function meta(cnf, millis, dimacs) {
  const m = el("div", "meta");
  m.append(
    el("span", "", `${cnf.names.length} variable${cnf.names.length === 1 ? "" : "s"}, ${cnf.varCount - cnf.names.length} auxiliary`),
    el("span", "", `${cnf.clauses.length} clauses`),
    el("span", "", `${millis < 1 ? "<1" : Math.round(millis)} ms`),
  );
  const toggle = el("button", "", "show CNF");
  toggle.type = "button";
  let pre = null;
  toggle.addEventListener("click", () => {
    if (pre) {
      pre.remove();
      pre = null;
      toggle.textContent = "show CNF";
    } else {
      pre = el("pre", "cnf", dimacs);
      m.after(pre);
      toggle.textContent = "hide CNF";
    }
  });
  m.append(toggle);
  return m;
}

// ───────────── main flow ─────────────
let generation = 0;
let debounce = null;
let slowTimer = null;

function autosize() {
  field.style.height = "auto";
  field.style.height = field.scrollHeight + "px";
}

async function run() {
  const text = field.value.trim();
  const gen = ++generation;
  clearTimeout(slowTimer);

  if (!text) {
    out.classList.remove("stale");
    out.replaceChildren();
    return;
  }

  let ast;
  try {
    ast = parse(text);
  } catch (e) {
    if (!(e instanceof FormulaError)) throw e;
    const where = el("code", "err-pos mono");
    where.append(text.slice(0, e.position), Object.assign(el("mark", "", "▸"), {}), text.slice(e.position));
    show("error", "Not a formula yet", el("div", "detail", e.message), where);
    return;
  }

  const cnf = tseitin(ast);
  const full = format(ast);
  const pretty = full.length > 60 ? "the formula" : full;

  if (cnf.constant !== null) {
    show(
      cnf.constant ? "sat" : "unsat",
      cnf.constant ? "Always true" : "Never true",
      el("div", "detail", `${cap(pretty)} simplifies to ${cnf.constant ? "⊤" : "⊥"} whatever the variables are.`),
    );
    return;
  }

  const dimacs = toDimacs(cnf);
  const negated = toDimacs(tseitin({ type: "not", a: ast }));

  slowTimer = setTimeout(() => {
    if (gen !== generation) return;
    const cancel = el("button", "", "cancel");
    cancel.type = "button";
    cancel.addEventListener("click", () => {
      generation++;
      cancelAll();
      show("", "Cancelled");
    });
    const m = el("div", "meta");
    m.append(el("span", "", "still working…"), cancel);
    show("pending", "Solving", m);
  }, 400);

  const started = performance.now();
  const [res, resNeg] = await Promise.all([solve(dimacs), solve(negated)]);
  if (gen !== generation) return;
  clearTimeout(slowTimer);
  const millis = performance.now() - started;

  if (res.result.status === "error") {
    show("error", "The solver refused the formula", el("div", "detail", res.result.error));
    return;
  }

  if (res.result.status === "unsat") {
    show(
      "unsat",
      "Never true",
      el("div", "detail", `No assignment satisfies ${pretty}. Its negation is a tautology.`),
      meta(cnf, millis, dimacs),
    );
    return;
  }

  const always = resNeg.result.status === "unsat";
  const detail = el("div", "detail");
  if (always) {
    detail.textContent = `${cap(pretty)} holds for every assignment; it is a tautology. One example:`;
  } else {
    detail.textContent = `${cap(pretty)} is true for the assignment below, but not for every one.`;
  }
  show("sat", always ? "Always true" : "Satisfiable", detail, assignment(cnf.names, res.result.model), meta(cnf, millis, dimacs));
}

field.addEventListener("input", () => {
  autosize();
  out.classList.add("stale");
  clearTimeout(debounce);
  debounce = setTimeout(run, 250);
});
field.addEventListener("keydown", (e) => {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    clearTimeout(debounce);
    run();
  }
});

function insert(text, cursorOffset) {
  const { selectionStart: s, selectionEnd: e, value } = field;
  const needsSpaceBefore = s > 0 && !/\s|\(/.test(value[s - 1]) && text !== "()";
  const snippet = (needsSpaceBefore ? " " : "") + text + (text === "()" ? "" : " ");
  field.setRangeText(snippet, s, e, "end");
  if (cursorOffset !== undefined) {
    const pos = s + (needsSpaceBefore ? 1 : 0) + cursorOffset;
    field.setSelectionRange(pos, pos);
  }
  field.focus();
  field.dispatchEvent(new Event("input"));
}

for (const b of document.querySelectorAll("#symbols [data-insert]")) {
  b.addEventListener("click", () => insert(b.dataset.insert, b.dataset.cursor ? Number(b.dataset.cursor) : undefined));
}
for (const b of document.querySelectorAll(".ex")) {
  b.addEventListener("click", () => {
    field.value = b.textContent;
    autosize();
    field.focus();
    clearTimeout(debounce);
    run();
  });
}

autosize();
