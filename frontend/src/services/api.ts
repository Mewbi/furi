import axios from 'axios';

type ShortUrlResponse = {
  original_url: string;
  short_url: string;
};

type UriMetricsResponse = {
  uri: string,
  req_total: number,
  req_time_series: UriRequestTimeSeries[],
  device_count: UriDeviceCount[],
  country_access: UriCountryCount[],
}

type UriRequestTimeSeries = {
  date: number,
  count: number,
}

type UriDeviceCount = {
  device_type: string,
  count: number
}

type UriCountryCount = {
  country: string,
  count: number,
}

type ErrorResponse = {
  error: true;
  message?: string;
};

type MayBeError<T> = T | ErrorResponse;

const API_URL = import.meta.env.VITE_API_URL;
const API_ANALYTICS_URL = import.meta.env.VITE_API_ANALYTICS_URL;

const apiRequest = axios.create({
  baseURL: `${import.meta.env.PROD ? 'https' : 'http'}://${API_URL}`,
  withCredentials: true,
});

const apiAnalyticsRequest = axios.create({
  baseURL: `${import.meta.env.PROD ? 'https' : 'http'}://${API_ANALYTICS_URL}`,
  withCredentials: true,
});


const shortUrl = async (url: string): Promise<MayBeError<ShortUrlResponse>> => { 
  const res = await apiRequest.post(`/url`, {url})
    .then((response) => response.data as ShortUrlResponse)
    .catch((err) => {
      const data :ErrorResponse = {
        error: true
      };
      if (err.response && err.response.data.message)
        data.message = err.response.data.message;
      return data
  });

  return res;
};

const getAnalytics = async (uri: string, from: number, to: number, aggregation: string): Promise<MayBeError<UriMetricsResponse>> => { 
  const res = await apiAnalyticsRequest.post(`/${uri}`, {aggregation, from, to})
    .then((response) => response.data as UriMetricsResponse)
    .catch((err) => {
      const data :ErrorResponse = {
        error: true
      };
      if (err.response && err.response.data.message)
        data.message = err.response.data.message;
      return data
  });

  return res;
};

export default { shortUrl, getAnalytics };
