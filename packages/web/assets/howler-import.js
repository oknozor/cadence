import { Howl } from "howler";

export const howli = {
  queue: (url) => {
    console.log("Queueing sound:", url);
    const sound = new Howl({
      src: [url],
      html5: true, // let the browser decode (helps with CORS)
      volume: 1,
      format: ["mp3", "ogg", "opus", "wav"], // accept whatever the server sends
      onload() {
        console.log("[howli] onload – success");
      },
      onloaderror(id, e) {
        console.error("[howli] onloaderror", e);
      },
      onplay() {
        console.log("[howli] onplay – really playing");
      },
      onplayerror(id, e) {
        console.error("[howli] onplayerror", e);
      },
      onend() {
        console.log("[howli] onend");
      },
      onstop() {
        console.log("[howli] onstop");
      },
    });
    console.log("[howli] Howl volume reports", sound.volume());
    sound.load;
    sound
      .play()
      .then(() => {
        console.log("[howli] play – success");
      })
      .catch((error) => {
        console.error("[howli] play – error", error);
      });
  },
};

window.howli = howli;
