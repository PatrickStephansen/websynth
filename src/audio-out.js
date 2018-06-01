const audioContext = new window.AudioContext()

const oscillator = audioContext.createOscillator();

oscillator.type = 'sine';
oscillator.frequency.setValueAtTime(440, audioContext.currentTime); // value in hertz
oscillator.connect(audioContext.destination);
oscillator.start();