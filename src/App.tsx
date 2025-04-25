import "./App.scss";
import Router from "./app/router";

function App() {

  // async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
      <Router />
    </main>
  );
}

export default App;
