type ProfileProps = {
	profilePicture: string;
	displayName: string;
	username: string;
	bio: string;
};

const ProfileInfo = ({
	profilePicture,
	displayName,
	username,
	bio,
}: ProfileProps) => {
	return (
		<div className="profile-header">
			<img
				src={profilePicture}
				alt={`${displayName}'s profile`}
				className="profile-picture"
			/>
			<h1 className="displayname">{displayName}</h1>
			<h2 className="username">@{username}</h2>
			<p className="bio">{bio}</p>
		</div>
	);
};

export default ProfileInfo;