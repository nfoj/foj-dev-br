$('.btn').click(function () {
    $(this).toggleClass("click");
    $('.sidebar').toggleClass("show");
});

$('.feat-btn').click(function () {
    $('aside ul .feat-show').toggleClass("show")
});

$('.serv-btn').click(function () {
    $('aside ul .serv-show').toggleClass("show1")
});