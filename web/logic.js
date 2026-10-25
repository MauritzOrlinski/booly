// Propositional formula parser and Tseitin transformation to CNF.
//
// Grammar (lowest precedence first):
//   iff   := imp ( '<->' imp )*          left associative
//   imp   := or  ( '->'  imp )?          right associative
//   or    := xor ( '|'   xor )*
//   xor   := and ( '^'   and )*
//   and   := not ( '&'   not )*
//   not   := '!' not | atom
//   atom  := variable | 'true' | 'false' | '(' iff ')'
//
// Several spellings are accepted for every operator, see TOKEN_SPEC.

export class FormulaError extends Error {
  constructor(message, position) {
    super(message);
    this.position = position;
  }
}

const TOKEN_SPEC = [
  ["ws", /^\s+/],
  ["iff", /^(<->|<=>|↔|≡|iff\b|xnor\b)/i],
  ["imp", /^(->|=>|→|⇒|⊃|implies\b)/i],
  ["xor", /^(\^|⊕|xor\b)/i],
  ["or", /^(\|\||\||∨|or\b|\+)/i],
  ["and", /^(&&|&|∧|and\b|\*|·)/i],
  ["not", /^(!|~|¬|not\b|-)/i],
  ["lparen", /^\(/],
  ["rparen", /^\)/],
  ["true", /^(true\b|⊤|1(?![\w']))/i],
  ["false", /^(false\b|⊥|0(?![\w']))/i],
  ["var", /^[A-Za-z_][A-Za-z0-9_']*/],
];

export function tokenize(text) {
  const tokens = [];
  let pos = 0;
  while (pos < text.length) {
    const rest = text.slice(pos);
    let matched = false;
    for (const [type, re] of TOKEN_SPEC) {
      const m = rest.match(re);
      if (!m) continue;
      if (type !== "ws") tokens.push({ type, text: m[0], pos });
      pos += m[0].length;
      matched = true;
      break;
    }
    if (!matched) throw new FormulaError(`Unexpected character "${text[pos]}"`, pos);
  }
  tokens.push({ type: "end", text: "", pos });
  return tokens;
}

export function parse(text) {
  const tokens = tokenize(text);
  let i = 0;
  const peek = () => tokens[i];
  const next = () => tokens[i++];
  const expect = (type, what) => {
    const t = peek();
    if (t.type !== type) throw new FormulaError(`Expected ${what} but found ${describe(t)}`, t.pos);
    return next();
  };
  const describe = (t) => (t.type === "end" ? "the end of the formula" : `"${t.text}"`);

  function parseIff() {
    let left = parseImp();
    while (peek().type === "iff") {
      next();
      left = { type: "iff", a: left, b: parseImp() };
    }
    return left;
  }
  function parseImp() {
    const left = parseOr();
    if (peek().type === "imp") {
      next();
      return { type: "imp", a: left, b: parseImp() };
    }
    return left;
  }
  function parseOr() {
    let left = parseXor();
    while (peek().type === "or") {
      next();
      left = { type: "or", a: left, b: parseXor() };
    }
    return left;
  }
  function parseXor() {
    let left = parseAnd();
    while (peek().type === "xor") {
      next();
      left = { type: "xor", a: left, b: parseAnd() };
    }
    return left;
  }
  function parseAnd() {
    let left = parseNot();
    while (peek().type === "and") {
      next();
      left = { type: "and", a: left, b: parseNot() };
    }
    return left;
  }
  function parseNot() {
    if (peek().type === "not") {
      next();
      return { type: "not", a: parseNot() };
    }
    return parseAtom();
  }
  function parseAtom() {
    const t = peek();
    switch (t.type) {
      case "var":
        next();
        return { type: "var", name: t.text };
      case "true":
        next();
        return { type: "const", value: true };
      case "false":
        next();
        return { type: "const", value: false };
      case "lparen": {
        next();
        const inner = parseIff();
        expect("rparen", '")"');
        return inner;
      }
      case "end":
        throw new FormulaError(
          i === 0 ? "The formula is empty" : "The formula ends too early, an operand is missing",
          t.pos,
        );
      case "rparen":
        throw new FormulaError('Unmatched ")"', t.pos);
      default:
        throw new FormulaError(`Expected a variable or "(" but found ${describe(t)}`, t.pos);
    }
  }

  const ast = parseIff();
  const t = peek();
  if (t.type !== "end") {
    throw new FormulaError(
      t.type === "rparen" ? 'Unmatched ")"' : `Unexpected ${describe(t)}, an operator is missing`,
      t.pos,
    );
  }
  return ast;
}

/** Replace sub-formulas that are constant, so Tseitin never sees true/false. */
export function fold(node) {
  if (node.type === "var" || node.type === "const") return node;
  if (node.type === "not") {
    const a = fold(node.a);
    if (a.type === "const") return { type: "const", value: !a.value };
    if (a.type === "not") return a.a;
    return { type: "not", a };
  }
  const a = fold(node.a);
  const b = fold(node.b);
  const ca = a.type === "const" ? a.value : null;
  const cb = b.type === "const" ? b.value : null;
  const not = (x) => fold({ type: "not", a: x });
  switch (node.type) {
    case "and":
      if (ca === false || cb === false) return { type: "const", value: false };
      if (ca === true) return b;
      if (cb === true) return a;
      break;
    case "or":
      if (ca === true || cb === true) return { type: "const", value: true };
      if (ca === false) return b;
      if (cb === false) return a;
      break;
    case "imp":
      if (ca === false || cb === true) return { type: "const", value: true };
      if (ca === true) return b;
      if (cb === false) return not(a);
      break;
    case "xor":
      if (ca !== null && cb !== null) return { type: "const", value: ca !== cb };
      if (ca === false) return b;
      if (cb === false) return a;
      if (ca === true) return not(b);
      if (cb === true) return not(a);
      break;
    case "iff":
      if (ca !== null && cb !== null) return { type: "const", value: ca === cb };
      if (ca === true) return b;
      if (cb === true) return a;
      if (ca === false) return not(b);
      if (cb === false) return not(a);
      break;
  }
  return { type: node.type, a, b };
}

/** Collect variable names in order of first appearance. */
export function variables(node, out = []) {
  if (node.type === "var") {
    if (!out.includes(node.name)) out.push(node.name);
  } else if (node.type === "not") variables(node.a, out);
  else if (node.type !== "const") {
    variables(node.a, out);
    variables(node.b, out);
  }
  return out;
}

/**
 * Tseitin transformation. Input variables get the ids 1..n (order of first
 * appearance), every compound sub-formula gets a fresh id above n.
 * Returns { names, clauses, varCount, constant } where `constant` is set
 * when the formula folds to true/false and no CNF is needed.
 */
export function tseitin(node) {
  const folded = fold(node);
  const names = variables(folded);
  if (folded.type === "const") return { names, clauses: [], varCount: names.length, constant: folded.value };

  const ids = new Map(names.map((n, i) => [n, i + 1]));
  let fresh = names.length;
  const clauses = [];
  // Drop duplicate literals and skip clauses that contain both l and ¬l (always true).
  const add = (clause) => {
    const lits = [...new Set(clause)];
    if (lits.some((l) => lits.includes(-l))) return;
    clauses.push(lits);
  };

  function lit(n) {
    switch (n.type) {
      case "var":
        return ids.get(n.name);
      case "not":
        return -lit(n.a);
      case "const": {
        const x = ++fresh;
        add([n.value ? x : -x]);
        return x;
      }
    }
    const a = lit(n.a);
    const b = lit(n.b);
    const x = ++fresh;
    switch (n.type) {
      case "and": // x <-> a & b
        add([-x, a]); add([-x, b]); add([x, -a, -b]);
        break;
      case "or": // x <-> a | b
        add([-x, a, b]); add([x, -a]); add([x, -b]);
        break;
      case "imp": // x <-> (!a | b)
        add([-x, -a, b]); add([x, a]); add([x, -b]);
        break;
      case "xor": // x <-> a ^ b
        add([-x, a, b]); add([-x, -a, -b]); add([x, -a, b]); add([x, a, -b]);
        break;
      case "iff": // x <-> (a <-> b)
        add([-x, -a, b]); add([-x, a, -b]); add([x, a, b]); add([x, -a, -b]);
        break;
    }
    return x;
  }

  const root = lit(folded);
  add([root]);
  return { names, clauses, varCount: fresh, constant: null };
}

export function toDimacs({ names, clauses, varCount }) {
  const lines = names.map((n, i) => `c var ${i + 1} = ${n}`);
  lines.push(`p cnf ${varCount} ${clauses.length}`);
  for (const c of clauses) lines.push(c.join(" ") + " 0");
  return lines.join("\n") + "\n";
}

/** Pretty-print an AST with the Unicode operators and minimal parentheses. */
export function format(node) {
  const prec = { iff: 1, imp: 2, or: 3, xor: 4, and: 5, not: 6, var: 7, const: 7 };
  const sym = { iff: " ↔ ", imp: " → ", or: " ∨ ", xor: " ⊕ ", and: " ∧ " };
  // minPrec: the lowest precedence a child may have without parentheses
  const go = (n, minPrec) => {
    let s;
    if (n.type === "var") s = n.name;
    else if (n.type === "const") s = n.value ? "⊤" : "⊥";
    else if (n.type === "not") s = "¬" + go(n.a, prec.not);
    else {
      const p = prec[n.type];
      // implication is right associative, everything else left associative
      s = go(n.a, n.type === "imp" ? p + 1 : p) + sym[n.type] + go(n.b, n.type === "imp" ? p : p + 1);
    }
    return prec[n.type] < minPrec ? `(${s})` : s;
  };
  return go(node, 0);
}
