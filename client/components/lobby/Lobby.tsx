import React, { useState } from "react";
import { useNavigate } from "react-router-dom"; // Import useNavigate

import "../App.css";

const LobbyPage = () => {
	const [players, setPlayers] = useState([
		"Player1",
		"Player2",
		"Player3", // ...add more player names
	]);
	const [lobbyLink, setLobbyLink] = useState("http://example.com/lobby");
	const [showPlayers, setShowPlayers] = useState(false);
	const navigate = useNavigate(); // Initialize the navigate function

	const copyLobbyLink = () => {
		navigator.clipboard.writeText(lobbyLink);
		alert("Link copied to clipboard!");
	};

	const togglePlayerList = () => setShowPlayers(!showPlayers);

	const startGame = () => {
		// Placeholder for start game logic
		alert("Starting the game with " + players.length + " players.");
		navigate("/timeline");
	};

	return (
		<div className="app">
			<div className="lobby-container">
				<div className="lobby-link-box">
					<span className="lobby-link">{lobbyLink}</span>
					<button onClick={copyLobbyLink} className="copy-button">
						Copy Link
					</button>
				</div>
				<div className="player-count" onClick={togglePlayerList}>
					Players: {players.length}
				</div>
				{showPlayers && (
					<div className="player-list">
						{players.map((player, index) => (
							<div key={index} className="player-name">
								{player}
							</div>
						))}
					</div>
				)}
				<button onClick={startGame} className="start-game-button">
					Start Game
				</button>
			</div>
		</div>
	);
};

export default LobbyPage;
