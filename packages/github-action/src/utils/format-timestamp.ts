function getTimezoneOffset(timezone: string): string {
  if (timezone === 'UTC') return '';

  try {
    const now = new Date();
    const utcDate = new Date(now.toLocaleString('en-US', { timeZone: 'UTC' }));
    const tzDate = new Date(now.toLocaleString('en-US', { timeZone: timezone }));
    const offset = (tzDate.getTime() - utcDate.getTime()) / (1000 * 60 * 60);

    if (offset === 0) return '';
    const sign = offset > 0 ? '+' : '';
    return `${sign}${offset}`;
  } catch {
    return '';
  }
}

export function formatTimestamp(timezone: string): string {
  const now = new Date();

  try {
    const year = now.toLocaleString('en-US', { timeZone: timezone, year: 'numeric' });
    const month = now.toLocaleString('en-US', { timeZone: timezone, month: '2-digit' });
    const day = now.toLocaleString('en-US', { timeZone: timezone, day: '2-digit' });
    const time = now.toLocaleString('en-GB', {
      timeZone: timezone,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    });

    const offset = getTimezoneOffset(timezone);
    return `${year}-${month}-${day} ${time} (UTC${offset})`;
  } catch {
    return `${now.toISOString()} (UTC)`;
  }
}
