import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import BottomNav from "../common/BottomNav";
import Post from "../common/Post";

function FeedPage() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		setGreetMsg(await invoke("greet", { name }));
	}

	const posts = [
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
		{
			displayName: "Jerome PogChampion",
			username: "JeromeJorganson",
			timePosted: "2s",
			content:
				"lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
		},
	];

	return (
		<div className="app">
			<div className="feed">
				{posts.map((msg, index) => (
					<Post
						key={index}
						displayName={msg.displayName}
						username={msg.username}
						timePosted={msg.timePosted}
						content={msg.content}
					/>
				))}
			</div>
			<BottomNav />
		</div>
	);
}

export default FeedPage;
