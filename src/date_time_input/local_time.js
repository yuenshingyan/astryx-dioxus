const now = new Date();
return [now.getHours(), now.getMinutes(), now.getSeconds()]
  .map(part => String(part).padStart(2, '0')).join(':');
