import { useState } from "react";
import { useNavigate } from "react-router-dom"; // Import useNavigate

const LoginPage = () => {
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const navigate = useNavigate(); // Initialize the navigate function

	const handleLogin = () => {
		alert("Login attempt with: " + username + " " + password);

		navigate("/lobby");
	};

	return (
		<div className="app">
			<div className="login-container">
				<div className="login-form">
					<input
						type="text"
						placeholder="Username"
						value={username}
						onChange={(e) => setUsername(e.target.value)}
						className="login-input"
					/>
					<input
						type="password"
						placeholder="Password"
						value={password}
						onChange={(e) => setPassword(e.target.value)}
						className="login-input"
					/>
					<button onClick={handleLogin} className="login-button">
						Login
					</button>
				</div>
			</div>
		</div>
	);
};

export default LoginPage;
