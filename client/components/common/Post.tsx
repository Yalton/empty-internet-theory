import ProfileIcon from "./ProfileIcon";
import IconButton from "@mui/material/IconButton";
import FavoriteBorderIcon from "@mui/icons-material/FavoriteBorder";
import RepeatIcon from "@mui/icons-material/Repeat"; // for retweet button

type PostProps = {
	displayName: string;
	username: string;
	timePosted: string;
	content: string;
};

const Post = ({ displayName, username, content, timePosted }: PostProps) => (
	<>
		<hr className="post-divider" />
		<div className="post">
			<div className="post-header">
				<ProfileIcon />
				<div className="post-info">
					<span className="displayname">{displayName}</span>
					<span className="username">@{username}</span>
					<span className="timestamp">{timePosted}</span>
				</div>
			</div>
			<div className="post-content">{content}</div>
			<div className="post-actions">
				<IconButton aria-label="like" sx={{ color: "var(--color-off-white)" }}>
					<FavoriteBorderIcon />
				</IconButton>
				<IconButton
					aria-label="retweet"
					sx={{ color: "var(--color-off-white)" }}
				>
					<RepeatIcon />
				</IconButton>
			</div>
		</div>
	</>
);

export default Post;
