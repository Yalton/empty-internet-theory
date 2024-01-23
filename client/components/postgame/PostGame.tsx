import React from "react";
import "../App.css";

// Mock data for player scores
const playerScores = [
	{ username: "PlayerOne", points: 150 },
	{ username: "PlayerTwo", points: 200 },
	// ... add more players as needed
];

const PostGame = () => {
	return (
		<div className="end-screen">
			<h1 className="end-screen-title">Game Over</h1>
			<div className="scoreboard">
				{playerScores.map((player, index) => (
					<div key={index} className="score-entry">
						<span className="username">{player.username}</span>
						<span className="points">{player.points}</span>
					</div>
				))}
			</div>
		</div>
	);
};

export default PostGame;
