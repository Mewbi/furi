import axios from 'axios';

type ShortUrlResponse = {
  original_url: string;
  short_url: string;
};

type ErrorResponse = {
  error: true;
  message?: string;
};

type MayBeError<T> = T | ErrorResponse;

const API_URL = import.meta.env.VITE_API_URL;

const apiRequest = axios.create({
  baseURL: `${import.meta.env.PROD ? 'https' : 'http'}://${API_URL}`,
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

export default { shortUrl };
