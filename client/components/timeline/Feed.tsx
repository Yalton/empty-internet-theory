// const Post = ({userImage, tagUsername, displayUsername, messageContent}: PostProps) => {
//     return (
//         <div className="p-4 bg-white shadow rounded-lg mb-4">
//             <div className="flex items-center space-x-4">
//                 <img className="h-12 w-12 rounded-full" src={userImage} alt={tagUsername}/>
//                 <div>
//                     <h2 className="font-bold text-lg">{displayUsername}</h2>
//                     <h3 className="text-gray-500">@{tagUsername}</h3>
//                 </div>
//             </div>
//             <p className="mt-2 text-gray-600">{messageContent}</p>
//             <div className="mt-4 flex justify-between">
//                 <span className="text-gray-500">{'5s ago'}</span>
//                 <button className="bg-blue-500 text-white px-4 py-2 rounded">Reply</button>
//             </div>
//         </div>
//     );
// };

// import {useRef} from "react";
// import {useVirtualizer} from "@tanstack/react-virtual";

// const Feed2 = ({posts}: FeedProps) => {
//     const parentRef = useRef<HTMLDivElement>(null);
//
//     const virtualizer = useVirtualizer({
//         count: posts.length,
//         getScrollElement: () => parentRef.current,
//         estimateSize: () => 45,
//     });
//
//     const items = virtualizer.getVirtualItems();
//
//     return (
//         <>
//             <div ref={parentRef} className="w-96 h-96 overflow-auto">
//                 <div
//                     className={`relative w-full h-[${virtualizer.getTotalSize()}px]`}
//                 ></div>
//             </div>
//         </>
//     );
// };

import Post, {PostProps} from "@components/timeline/Post.tsx";

export interface FeedProps {
    posts: PostProps[];
}

const Feed = ({posts}: FeedProps) => {
    return (
        <div className={"mx-auto w-full flex justify-around"}>
            <div className={"flex flex-col gap-4 max-w-xl"}>
                {posts.map((post) => {
                    return (
                        <Post
                            userImage={post.userImage}
                            tagUsername={post.tagUsername}
                            displayUsername={post.displayUsername}
                            messageContent={post.messageContent}
                        />
                    );
                })}
            </div>
        </div>
    );
};

export default Feed;
