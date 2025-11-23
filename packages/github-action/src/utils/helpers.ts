export function pluralize(count: number, singular: string): string {
  return count === 1 ? singular : `${singular}s`;
}

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
    const formatted = now.toLocaleString('en-US', {
      timeZone: timezone,
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false,
    });

    const offset = getTimezoneOffset(timezone);
    return `${formatted} (UTC${offset})`;
  } catch {
    return `${now.toISOString()} (UTC)`;
  }
}
