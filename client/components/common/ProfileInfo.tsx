import React from "react";
import ProfileIcon from "./ProfileIcon";
import "../App.css";

type ProfileProps = {
	profilePicture: string;
	displayName: string;
	username: string;
	bio: string;
};

const ProfileInfo = ({ profilePicture, displayName, username, bio }: ProfileProps) => {
  return (
    <div className="profile-header">
      <ProfileIcon/>
      <div className="profile-details">
        <h1 className="displayname">{displayName}</h1>
        <h2 className="username">@{username}</h2>
        <p className="bio">{bio}</p>
      </div>
    </div>
  );
};

export default ProfileInfo;