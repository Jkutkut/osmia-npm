const CODE = `
Osmia {{ _OSMIA_VERSION }} - npm {{ _OSMIA_NPM_VERSION }}

Hello {{ user.name }}!`.trim()

const CTX = `{
  "user": {
    "name": "marvin"
  }
}`.trim();

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("code").value = CODE;
  document.getElementById("ctx").value = CTX;

  chkCtx.addEventListener("change", () => {
      ctxContainer.classList.toggle("hide", !chkCtx.checked);
  });
})

import('/pkg/osmia_npm.js').then(async (osmia) => {
  await osmia.default();
  return osmia;
}).then((osmia) => {
  const codeContainer = document.getElementById("code");
  const ctxContainer = document.getElementById("ctx");
  const resultContainer = document.getElementById("output");
  const btn = document.getElementById("run");
  const chkCtx = document.getElementById("chkCtx");

  const runCode = () => {
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
  btn.addEventListener("click", runCode);
}).catch(() => {
    const errorMsg = "Error loading wasm. Did you build?";
    console.error(errorMsg);
    document.getElementById("output").innerHTML = errorMsg;
});
