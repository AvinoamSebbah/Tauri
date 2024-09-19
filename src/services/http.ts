import { makeUnifiedNetwork } from 'unified-network';
import { fetch, ResponseType } from '@tauri-apps/api/http';
interface RequestParams {
  method: string;
  url: string;
  body: any;
  headers: Record<string, string>;
}

let requestProcesor : any = undefined;
// @ts-ignore
if (import.meta.env.VITE_NETWORK_PROCESSOR === 'tauri') {
  requestProcesor = async ({ method, url, body, headers } : RequestParams) => {
    let responseStatus = undefined;
    let responseData : any = undefined;
    let responseHeaders = undefined;

    const response = await fetch(url, {
      method: method.toUpperCase() as any,
      body,
      headers,
      responseType: ResponseType.Text,
    });

    responseStatus = response.status;
    responseHeaders = response.headers;
    responseData = response.data;

    if (
      response.ok &&
      responseHeaders['content-type']
        ?.toLowerCase()
        .includes('application/json')
    ) {
      try {
        responseData = JSON.parse(responseData);
      } catch (error : any) {
        throw new Error('could not parse response data ' + error.message);
      }
    }

    return {
      status: responseStatus,
      headers: responseHeaders,
      data: responseData,
    };
  };
}

export const $http = makeUnifiedNetwork({
  url: 'your-url-here',
  processor: requestProcesor,
});

export function generalHandleHttp(status: number, data: { message?: string }): boolean {
  if (status !== 200) {
    console.error(data?.message);
    return true;
  }
  return false;
}
