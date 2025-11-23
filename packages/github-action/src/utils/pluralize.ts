export function pluralize(count: number, singular: string): string {
  return count === 1 ? singular : `${singular}s`;
}
