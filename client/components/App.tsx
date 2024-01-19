import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import reactLogo from "@assets/react.svg";
import viteLogo from "@assets/vite.svg";
import tauriLogo from "@assets/tauri.svg";
import Pikachu from "@assets/image.png";
import "./App.css";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		setGreetMsg(await invoke("greet", { name }));
	}

	return (
		<div className="container">
			<h1>Welcome to Tauri!</h1>

			<div className="row">
				<a href="https://vitejs.dev" target="_blank" rel="noreferrer">
					<img src={viteLogo} className="logo vite" alt="Vite logo" />
				</a>
				<a href="https://tauri.app" target="_blank" rel="noreferrer">
					<img src={tauriLogo} className="logo tauri" alt="Tauri logo" />
				</a>
				<a href="https://reactjs.org" target="_blank" rel="noreferrer">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
			</div>

			<p>Click on the Tauri, Vite, and React logos to learn more.</p>
			<div className={"flex justify-center items-center m-4"}>
				<img
					src={Pikachu}
					alt={"Pikachu surprised face"}
					className={"w-16 h-16 bg-sky-900"}
				/>
			</div>

			<form
				className="row"
				onSubmit={(e) => {
					e.preventDefault();
					greet();
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
				/>
				<button type="submit">Greet</button>
			</form>

			<p>{greetMsg}</p>
		</div>
	);
}

export default App;