import App from "./App.svelte";
import init from "substrait-playground";

const app = init().then(
  () =>
    new App({
      target: document.getElementById("app"),
      props: {},
    })
);

export default app;
