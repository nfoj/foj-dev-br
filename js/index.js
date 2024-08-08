$('.btn').click(function () {
    $(this).toggleClass("click");
    $('.sidebar').toggleClass("show");
});

$('.linux-btn').click(function () {
    $('aside ul .linux-show').toggleClass("show")
});

$('.rust-btn').click(function () {
    $('aside ul .rust-show').toggleClass("show1")
});