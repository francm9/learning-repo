import 'package:flutter/material.dart';
import 'utils/appcolors.dart';

class FooterElement extends StatelessWidget
{
  const FooterElement({super.key, required this.icon, required this.text, required this.onPressed, required this.focusIcon});

  final IconData icon;
  final String text;
  final bool focusIcon;
  final Function() onPressed;

  @override
  Widget build(BuildContext context) 
  {
    return Column
    (
      mainAxisAlignment: MainAxisAlignment.start,
      children: 
      [
        IconButton
        (
          icon: Icon(icon),
          color: focusIcon ? Colors.grey : Colors.white,
          iconSize: 38,
          onPressed: onPressed,
        ),
        Text
        (
          text, 
          style: TextStyle
          (
            color: focusIcon
                ? Colors.grey 
                : Colors.white
          )
        )
      ],
    );
  }
}

class Footer extends StatelessWidget
{
  const Footer({super.key, required this.focusIcon});

  final int focusIcon;

  @override
  Widget build(BuildContext context) 
  {
    return Container
    (
      color: AppColors.pink100,
      height: 95,
      child: Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: 
      [
        FooterElement
        (
          icon: Icons.home,
          text: 'Home', 
          onPressed: () 
          {
            debugPrint(ModalRoute.of(context)?.settings.name);
            ModalRoute.of(context)?.settings.name != '/' ? Navigator.pushNamed(context,'/') : null;
          },
          focusIcon: focusIcon == 0
        ),
        FooterElement
        (
          icon: Icons.water,
          text: 'Data', 
          onPressed: () 
          {
            debugPrint(ModalRoute.of(context)?.settings.name);
            ModalRoute.of(context)?.settings.name != '/' ? Navigator.pushNamed(context,'/') : null;
          },
          focusIcon: focusIcon == 2
        ),
        FooterElement
        (
          icon: Icons.settings, 
          text: 'Settings',
          onPressed: () 
          {
            debugPrint(ModalRoute.of(context)?.settings.name);
            ModalRoute.of(context)?.settings.name != '/second' ? Navigator.pushNamed(context,'/second') : null;
          },
          focusIcon: focusIcon == 1
        ),

      ],
    ),
    );
  }
}