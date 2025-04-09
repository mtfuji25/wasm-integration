import init, {
  compute_sha256,
  generate_primes,
} from "./wasm/pkg/wasm_integration.js";

async function setup() {
  await init();
}

window.hashInput = async function () {
  await setup();

  const input = document.getElementById("inputText").value;

  try {
    const hash = compute_sha256(input);
    document.getElementById("hashResult").textContent = `SHA-256: ${hash}`;
  } catch (err) {
    document.getElementById("hashResult").textContent = `Error: ${err}`;
  }
};

window.generatePrimes = async function () {
  await setup();

  const limit = parseInt(document.getElementById("primeLimit").value, 10);

  try {
    const primes = await generate_primes(limit);
    document.getElementById("primesResult").textContent = `Primes: ${primes
      .slice(0, 100)
      .join(", ")}${primes.length > 100 ? ", ..." : ""}`;
  } catch (err) {
    document.getElementById("primesResult").textContent = `Error: ${err}`;
  }
};
