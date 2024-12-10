var chart = echarts.init(
  null,
  input.theme,
  {
    renderer: 'svg',
    ssr: true,
    width: input.width,
    height: input.height
  });

console.log(input.chart);

chart.setOption({ animation: false, progressive: 0 });
chart.setOption(input.chart);
var svg = chart.renderToSVGString();
var result = {
  svg: svg
};
result;