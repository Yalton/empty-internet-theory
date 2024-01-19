import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ProfileIcon from "./ProfileIcon";
import BottomNav from "./BottomNav";

// import reactLogo from "@assets/react.svg";
// import viteLogo from "@assets/vite.svg";
// import tauriLogo from "@assets/tauri.svg";
import "./App.css";

type MessageProps = {
	displayName: string;
	username: string;
	timePosted: string;
	content: string;
};

const Message = ({
	displayName,
	username,
	content,
	timePosted,
}: MessageProps) => (
	<div className="message">
		<div className="message-header">
			<ProfileIcon />
			<div className="message-info">
				<span className="displayname">{displayName}</span>
				<span className="username">@{username}</span>
				<span className="timestamp">{timePosted}</span>
			</div>
		</div>
		<div className="message-content">{content}</div>
	</div>
);

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		setGreetMsg(await invoke("greet", { name }));
	}

	// Dummy data for the sake of display
	const message = {
		displayName: "Jerome PogChampion",
		username: "JeromeJorganson",
		timePosted: "2s",
		content:
			"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
	},;

	return (
		<div className="app">
			<div className="feed">
				{Array(3).fill(message).map((msg, index) => (
					<Message
						key={index}
						displayName={msg.displayName}
						username={msg.username}
						timePosted={msg.timePosted}
						content={msg.content}
					/>
				))}
				<hr />
			</div>
			<BottomNav />
		</div>
	);
}

export default App;
