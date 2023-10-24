import "@kit/styles";

// javascript is supported
import "./App.css";

import {VNode, render} from "@hydrophobefireman/ui-lib";

import {Router} from "./_router";

function App(): VNode {
  return (
    <main>
      <Router />
    </main>
  );
}

render(<App />, document.getElementById("app-mount"));
