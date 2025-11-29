const module = await import('./module');
import('lodash').then((m) => m.default);
const lazyLoad = () => import('./component');
