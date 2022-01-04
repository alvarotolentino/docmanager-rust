import { check } from 'k6';
import http from 'k6/http';

export let options = {
  VUS: 1000,
  rate: 10000,
  timeUnit: '1s',
  duration: '1m',
};
export function post_testing() {
  const payload = '{"name":"User","birth_date":"1981-03-15","custom_data":{"random":4}}';
  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  http.post('http://127.0.0.1:8080/v1/user/', payload, params);
}

export default function () {
  post_testing();
}
