// import rust from "./pkg/glissando_synth_bg.wasm";
import * as rust from "./pkg/glissando_synth.js";

var chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

// decoder
// [https://gist.github.com/1020396] by [https://github.com/atk]
function atob(input) {
  var str = String(input).replace(/[=]+$/, ""); // #31: ExtendScript bad parse of /=
  if (str.length % 4 === 1) {
    throw new InvalidCharacterError(
      "'atob' failed: The string to be decoded is not correctly encoded."
    );
  }
  for (
    // initialize result and counters
    var bc = 0, bs, buffer, idx = 0, output = "";
    // get next character
    (buffer = str.charAt(idx++)); // eslint-disable-line no-cond-assign
    // character found in table? initialize bit storage and add its ascii value;
    ~buffer &&
    ((bs = bc % 4 ? bs * 64 + buffer : buffer),
    // and if not first of each 4 characters,
    // convert the first 8 bits to one ascii character
    bc++ % 4)
      ? (output += String.fromCharCode(255 & (bs >> ((-2 * bc) & 6))))
      : 0
  ) {
    // try to find character in table (0-63, not found => -1)
    buffer = chars.indexOf(buffer);
  }
  return output;
}

globalThis.atob = atob;

async function run() {
  console.log(rust);
  // const module = await rust({});
  // console.log(module);
  // globalThis.process = module.instance.exports.process;
}

class GlissandoSynth extends AudioWorkletProcessor {
  process(inputs, outputs, parameters) {
    // if (!globalThis.process) {
    //   return true;
    // }

    // console.log(globalThis.process());

    const output = outputs[0];
    output.forEach((channel) => {
      for (let i = 0; i < channel.length; i++) {
        channel[i] = Math.random() * 2 - 1;
      }
      // globalThis.process(channel);
    });

    return true;
  }
}

registerProcessor("glissando-synth", GlissandoSynth);

run();
