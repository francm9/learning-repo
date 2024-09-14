import 'package:flutter/material.dart';
import 'utils/appcolors.dart';

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
      height: 100,
      child: Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: 
      [
        IconButton
        (
          icon: const Icon(Icons.home),
          color: focusIcon == 0 ? Colors.grey : Colors.white,
          iconSize: 44,
          onPressed: () 
          {
            debugPrint(ModalRoute.of(context)?.settings.name);
            ModalRoute.of(context)?.settings.name != '/' ? Navigator.pushNamed(context,'/') : null;
          },
        ),

        IconButton
        (
          icon: const Icon(Icons.search),
          color: focusIcon == 1 ? Colors.grey : Colors.white,
          iconSize: 40,
          onPressed: () 
          {
            debugPrint(ModalRoute.of(context)?.settings.name);
            ModalRoute.of(context)?.settings.name != '/second' ? Navigator.pushNamed(context,'/second') : null;
          },
        ),

        IconButton
        (
          icon: const Icon(Icons.add_alarm),
          color: focusIcon == 2 ? Colors.grey : Colors.white,
          iconSize: 40,
          onPressed: () 
          {
            // Acción al hacer clic en el icono de configuración
          },
        ),

        IconButton
        (
          icon: const Icon(Icons.settings),
          color: focusIcon == 3 ? Colors.grey : Colors.white,
          iconSize: 40,
          onPressed: () 
          {
            // Acción al hacer clic en el icono de configuración
          },
        ),
      ],
    ),
    );
  }
}