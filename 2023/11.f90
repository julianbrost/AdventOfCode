program day11
  implicit none

  character(len=1024), dimension(1024) :: image
  integer(8) :: n, i, j, total, res1, res2, before, dist1, dist2, count
  logical :: empty

  image(:) = ""
  read (*, *, end=1) image(:)
1 n = 1024

  total = 0
  do i = 1, n
    do j = 1, n
      if (image(i)(j:j) == '#') then
        total = total + 1
      end if
    end do
  end do

  res1 = 0
  res2 = 0

  before = 0
  dist1 = 0
  dist2 = 0
  do i = 1, n
    count = 0
    do j = 1, n
      if (image(i)(j:j) == '#') then
        count = count + 1
      end if
    end do

    if (count > 0) then
      dist1 = dist1 + 1
      dist2 = dist2 + 1
    else
      dist1 = dist1 + 2
      dist2 = dist2 + 1000000
    end if

    res1 = res1 - (total - count - before) * dist1 * count + before * dist1 * count
    res2 = res2 - (total - count - before) * dist2 * count + before * dist2 * count
    before = before + count
  end do

  before = 0
  dist1 = 0
  dist2 = 0
  do j = 1, n
    count = 0
    do i = 1, n
      if (image(i)(j:j) == '#') then
        count = count + 1
      end if
    end do

    if (count > 0) then
      dist1 = dist1 + 1
      dist2 = dist2 + 1
    else
      dist1 = dist1 + 2
      dist2 = dist2 + 1000000
    end if

    res1 = res1 - (total - count - before) * dist1 * count + before * dist1 * count
    res2 = res2 - (total - count - before) * dist2 * count + before * dist2 * count
    before = before + count
  end do

  print *, "Part 1:", res1
  print *, "Part 2:", res2
end program day11
