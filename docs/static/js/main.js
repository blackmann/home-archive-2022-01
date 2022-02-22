
let contentHideTimeout;

function setUpResponsiveNavigation() {
  const menuBtn = document.querySelector('nav>.more')
  menuBtn.addEventListener('click', (event) => {
    document.querySelector(`.${menuBtn.dataset.target}`).classList.toggle('active')

    const isActive = menuBtn.classList.contains('active');

    menuBtn.classList.toggle('active')
    menuBtn.innerText = isActive  ? 'More' : 'Close'

    clearTimeout(contentHideTimeout)

    contentHideTimeout = setTimeout(() => {
      document.querySelector(`.${menuBtn.dataset.content}`).classList.toggle('hide')
    }, isActive ? 0 : 240)
  })
}

setUpResponsiveNavigation()