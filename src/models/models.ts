export interface RequestResponse {
	body: string;
	headers: string;
	status: string;
}

export interface RequestTab {
	uuid: string;
	data: Tabdata;
	saved_data: Tabdata | undefined;
}

export interface RequestParameter {
	uuid: string;
	enabled: boolean;
	key: string;
	value: string;
}

export interface Tabdata {
	name: string;
	url: string;
	response: RequestResponse | undefined;
	parameters: RequestParameter[];
}
