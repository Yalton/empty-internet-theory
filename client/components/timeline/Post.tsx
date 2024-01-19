export interface PostProps {
    userImage: string,
    tagUsername: string;
    displayUsername: string;
    messageContent: string;
    // timestamp: number,
}

const Post = ({userImage, tagUsername, displayUsername, messageContent}: PostProps) => {
    return (
        <div className={"flex flex-row"}>
            <div>
                <img className=" w-12 h-auto rounded-full" src={userImage} alt={tagUsername}/>
            </div>

            <div className={"flex flex-col"}>
                <div className={"flex gap-2"}>
                    <span className={"font-bold"}>{displayUsername}</span>
                    <span className={"text-gray-400"}>@{tagUsername} · {"2s ago"}</span>
                </div>
                <div>{messageContent}</div>
            </div>
        </div>
    );
};

export default Post
