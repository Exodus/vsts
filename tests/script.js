import http from 'k6/http';
import { check }  from 'k6';
import { sleep } from 'k6';
export const options = {
  vus: 50,
  duration: '30s',
};

const urls = {
  generate: "http://localhost:3030/gen",
  validate: "http://localhost:3030/validate/",
  link: "http://localhost:3030/auth"
}

export default function () {
  let res = http.get(urls.generate);
  sleep(1);
  check(res, {
    '200: JWT Generated': (r) => r.status === 200,
  });

  res = http.get(urls.validate + res.body)
  check(res, {
    '200: JWT Validated': (r) => r.status === 200,
  });

  // res = http.get(urls.validate + res.body)
  // check(res, {
  //   '200: JWT Validated': (r) => r.status === 200,
  // });
}