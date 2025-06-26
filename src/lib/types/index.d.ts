export type Comment = {
	content: string;
	id: number;
	meme_id: number;
	username: sting;
};

export type MemeWithUsernameAndCommentsCount = {
	comment_count: number;
	id: number;
	imageUrl: string;
	like_count: number;
	username: string;
};

export type MemeWithUsernameAndComments = {
	comments: String[];
	id: number;
	image_url: string;
	like_count: number;
	username: string;
};

export type User = {
	id: string;
	is_admin: boolean;
	username: string;
};
