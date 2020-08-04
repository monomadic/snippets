function internet_time(dt: Date) {
  return Math.floor((((dt.getUTCHours() + 1) % 24) + dt.getUTCMinutes() / 60 + dt.getUTCSeconds() / 3600) * 1000 / 24);
}

let dt: Date = new Date();
console.log("@" + internet_time(dt)); 
