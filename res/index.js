const CODE = `
Osmia {{ _OSMIA_VERSION }} - npm {{ _OSMIA_NPM_VERSION }}

Hello {{ user.name }}!`.trim()

const CTX = `{
  "user": {
    "name": "marvin"
  }
}`.trim();

document.addEventListener("DOMContentLoaded", () => {
  const codeContainer = document.getElementById("code");
  const ctxContainer = document.getElementById("ctx");
  const chkLiveCtx = document.getElementById("chkLiveCtx");
  const ctxOutput = document.getElementById("ctxOutput");
  const outputCtxContainer = document.getElementById("outputCtxContainer");

  codeContainer.value = CODE;
  ctxContainer.value = CTX;
  ctxOutput.innerHTML = "Update the context to update";

  chkCtx.addEventListener("change", () => {
      ctxContainer.classList.toggle("hide", !chkCtx.checked);
  });

  chkLiveCtx.addEventListener("change", () => {
    const isChecked = chkLiveCtx.checked;
    outputCtxContainer.classList.toggle("hide", !isChecked);
  });
});

const OFFSET = "  ";
const format_ctx_json_dump = (node) => {
  return convert_dump(node, {offset: "", inMethod: false});
};

const convert_dump = (node, options) => {
  let result = "";
  const {offset} = options;
  options = {...options, offset: offset + OFFSET};
  if (node.type == "object") {
    result += "{\n";
    const entries = Object.entries(node.value);
    entries.sort((a, b) => a < b ? -1 : 1);
    result += entries.map(([key, value]) => {
      if (key == "_method") {
        const methodOptions = {...options, inMethod: true};
        return `${offset + OFFSET}${key}: ${convert_dump(value, methodOptions)}`;
      }
      return `${offset + OFFSET}${key}: ${convert_dump(value, options)}`;
    }).join(",\n");
    result += "\n" + offset + "}";
  }
  else if (node.type == "array") {
    result += "[\n";
    result += node.elements.map((value) => {
      return `${offset}${convert_dump(value, options)}`;
    }).join(",\n");
    result += "\n" + offset + "]";
  }
  else if (node.type == "variable") {
    result = node.value;
  }
  else if (node.type == "function") {
    let args;
    if (node.arity) {
      const arity = options.inMethod ? node.arity - 1 : node.arity;
      args = Array(arity).fill("arg").map((e, idx) => e + idx).join(", ");
    }
    else {
      args = "...args";
    }
    result = `(${args}) => ${node.description}`;
  }
  return result;
};

import('../pkg/osmia_npm.js').then(async (osmia) => {
  await osmia.default();
  return osmia;
}).then((osmia) => {
  const codeContainer = document.getElementById("code");
  const ctxContainer = document.getElementById("ctx");
  const resultContainer = document.getElementById("output");
  const btn = document.getElementById("run");
  const chkCtx = document.getElementById("chkCtx");
  const ctxOutput = document.getElementById("ctxOutput");
  const outputContainer = document.getElementById("outputContainer");
  const outputCtxContainer = document.getElementById("outputCtxContainer");

  const runCode = () => {
    outputContainer.classList.remove("hide");
    const code = codeContainer.value;
    let result;
    try {
      result = chkCtx.checked ?
        osmia.run_ctx(ctxContainer.value, code) :
        osmia.run(code);
    }
    catch (e) {
      result = e;
    }
    console.table([
      {"variable": "code", "value": code},
      {"variable": "ctx", "value": chkCtx.checked ? ctxContainer.value : null},
      {"variable": "result", "value": result}
    ],["variable", "value"]);
    resultContainer.innerHTML = result;
  };
  const updateCurrentCtx = () => {
    if (outputCtxContainer.classList.contains("hide")) {
      return;
    }
    try {
      const dump = osmia.ctx_json_dump(ctxContainer.value);
      const formatted = format_ctx_json_dump(JSON.parse(dump));
      ctxOutput.innerHTML = formatted;
    }
    catch (e) {
      console.error(e);
      ctxOutput.innerHTML = "Invalid JSON context";
    }
  };

  btn.addEventListener("click", runCode);
  ctxContainer.addEventListener("input", updateCurrentCtx);
  updateCurrentCtx();
}).catch(() => {
  const errorMsg = "Error loading wasm. Did you build?";
  console.error(errorMsg);
  document.getElementById("output").innerHTML = errorMsg;
});
