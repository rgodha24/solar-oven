import { graph_data, Oven, GraphDataResponse, GraphData, oven_from_json } from 'solar-oven';
import type { Oven as JSONOven } from './types';

let data: GraphData[] | null;

export function getData(o: JSONOven) {
	const oven = oven_from_json(JSON.stringify(o));
	if (!oven) {
		return [];
	}

	let responseType: GraphDataResponse = GraphDataResponse.Tio;
	let reflectorML = 3.0;

	data?.forEach((d) => d.free());

	data = graph_data(oven, responseType, reflectorML);

	oven.free();

	return data;
}
