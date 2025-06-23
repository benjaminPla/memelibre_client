export type Meme = {
	id: number;
	imageUrl: string;
	created_by?: string;
};

export type User = {
	id: string;
	is_admin: boolean;
	username: string;
};
