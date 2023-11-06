import type { Payload } from './types';

export async function getGraphData(payload: Payload): Promise<[number, number, number][]> {
  let res = await fetch('/api/graph', {
    method: 'POST',
    body: JSON.stringify(payload)
  }).catch(console.error);

  return (await res?.json()) || [];
}
