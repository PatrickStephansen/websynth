import {  } from "websynth.gc";
export class SynthOutput extends AudioWorkletProcessor {
  constructor(options) {
    super(options);
  }

  process(inputs, outputs, parameters) {
    let output = outputs[0];
    const channels = outputs.length;


    for (let channel = 0; channel < output.length; channel++) {
      output[channel].set(getSynthOutputBuffer());
    }
    return true;
  }

}
registerProcessor('my-worklet-processor', SynthOutput);
