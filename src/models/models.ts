export interface RequestResponse {
	body: string;
	headers: string;
	status: string;
}

export interface RequestTab {
	uuid: string;
	name: string;
	url: string;
	response: RequestResponse | undefined;
	parameters: RequestParameter[];
}

export interface RequestParameter {
	uuid: string;
	enabled: boolean;
	key: string;
	value: string;
}